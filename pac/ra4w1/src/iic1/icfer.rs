#[doc = "Register `ICFER` reader"]
pub struct R(crate::R<ICFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICFER` writer"]
pub struct W(crate::W<ICFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ICFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMOE` reader - Timeout Function Enable"]
pub type TMOE_R = crate::BitReader<TMOE_A>;
#[doc = "Timeout Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMOE_A {
    #[doc = "0: The timeout function is disabled."]
    _0 = 0,
    #[doc = "1: The timeout function is enabled."]
    _1 = 1,
}
impl From<TMOE_A> for bool {
    #[inline(always)]
    fn from(variant: TMOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TMOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMOE_A {
        match self.bits {
            false => TMOE_A::_0,
            true => TMOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMOE_A::_1
    }
}
#[doc = "Field `TMOE` writer - Timeout Function Enable"]
pub type TMOE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICFER_SPEC, TMOE_A, O>;
impl<'a, const O: u8> TMOE_W<'a, O> {
    #[doc = "The timeout function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMOE_A::_0)
    }
    #[doc = "The timeout function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMOE_A::_1)
    }
}
#[doc = "Field `MALE` reader - Master Arbitration-Lost Detection Enable"]
pub type MALE_R = crate::BitReader<MALE_A>;
#[doc = "Master Arbitration-Lost Detection Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MALE_A {
    #[doc = "0: Master arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: Master arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<MALE_A> for bool {
    #[inline(always)]
    fn from(variant: MALE_A) -> Self {
        variant as u8 != 0
    }
}
impl MALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MALE_A {
        match self.bits {
            false => MALE_A::_0,
            true => MALE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MALE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MALE_A::_1
    }
}
#[doc = "Field `MALE` writer - Master Arbitration-Lost Detection Enable"]
pub type MALE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICFER_SPEC, MALE_A, O>;
impl<'a, const O: u8> MALE_W<'a, O> {
    #[doc = "Master arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MALE_A::_0)
    }
    #[doc = "Master arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MALE_A::_1)
    }
}
#[doc = "Field `NALE` reader - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NALE_R = crate::BitReader<NALE_A>;
#[doc = "NACK Transmission Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NALE_A {
    #[doc = "0: NACK transmission arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: NACK transmission arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<NALE_A> for bool {
    #[inline(always)]
    fn from(variant: NALE_A) -> Self {
        variant as u8 != 0
    }
}
impl NALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NALE_A {
        match self.bits {
            false => NALE_A::_0,
            true => NALE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NALE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NALE_A::_1
    }
}
#[doc = "Field `NALE` writer - NACK Transmission Arbitration-Lost Detection Enable"]
pub type NALE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICFER_SPEC, NALE_A, O>;
impl<'a, const O: u8> NALE_W<'a, O> {
    #[doc = "NACK transmission arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NALE_A::_0)
    }
    #[doc = "NACK transmission arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NALE_A::_1)
    }
}
#[doc = "Field `SALE` reader - Slave Arbitration-Lost Detection Enable"]
pub type SALE_R = crate::BitReader<SALE_A>;
#[doc = "Slave Arbitration-Lost Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SALE_A {
    #[doc = "0: Slave arbitration-lost detection is disabled."]
    _0 = 0,
    #[doc = "1: Slave arbitration-lost detection is enabled."]
    _1 = 1,
}
impl From<SALE_A> for bool {
    #[inline(always)]
    fn from(variant: SALE_A) -> Self {
        variant as u8 != 0
    }
}
impl SALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SALE_A {
        match self.bits {
            false => SALE_A::_0,
            true => SALE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SALE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SALE_A::_1
    }
}
#[doc = "Field `SALE` writer - Slave Arbitration-Lost Detection Enable"]
pub type SALE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICFER_SPEC, SALE_A, O>;
impl<'a, const O: u8> SALE_W<'a, O> {
    #[doc = "Slave arbitration-lost detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SALE_A::_0)
    }
    #[doc = "Slave arbitration-lost detection is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SALE_A::_1)
    }
}
#[doc = "Field `NACKE` reader - NACK Reception Transfer Suspension Enable"]
pub type NACKE_R = crate::BitReader<NACKE_A>;
#[doc = "NACK Reception Transfer Suspension Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKE_A {
    #[doc = "0: Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    _0 = 0,
    #[doc = "1: Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    _1 = 1,
}
impl From<NACKE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKE_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKE_A {
        match self.bits {
            false => NACKE_A::_0,
            true => NACKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKE_A::_1
    }
}
#[doc = "Field `NACKE` writer - NACK Reception Transfer Suspension Enable"]
pub type NACKE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICFER_SPEC, NACKE_A, O>;
impl<'a, const O: u8> NACKE_W<'a, O> {
    #[doc = "Transfer operation is not suspended during NACK reception (transfer suspension disabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKE_A::_0)
    }
    #[doc = "Transfer operation is suspended during NACK reception (transfer suspension enabled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKE_A::_1)
    }
}
#[doc = "Field `NFE` reader - Digital Noise Filter Circuit Enable"]
pub type NFE_R = crate::BitReader<NFE_A>;
#[doc = "Digital Noise Filter Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFE_A {
    #[doc = "0: No digital noise filter circuit is used."]
    _0 = 0,
    #[doc = "1: A digital noise filter circuit is used."]
    _1 = 1,
}
impl From<NFE_A> for bool {
    #[inline(always)]
    fn from(variant: NFE_A) -> Self {
        variant as u8 != 0
    }
}
impl NFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFE_A {
        match self.bits {
            false => NFE_A::_0,
            true => NFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NFE_A::_1
    }
}
#[doc = "Field `NFE` writer - Digital Noise Filter Circuit Enable"]
pub type NFE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICFER_SPEC, NFE_A, O>;
impl<'a, const O: u8> NFE_W<'a, O> {
    #[doc = "No digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFE_A::_0)
    }
    #[doc = "A digital noise filter circuit is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFE_A::_1)
    }
}
#[doc = "Field `SCLE` reader - SCL Synchronous Circuit Enable"]
pub type SCLE_R = crate::BitReader<SCLE_A>;
#[doc = "SCL Synchronous Circuit Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCLE_A {
    #[doc = "0: No SCL synchronous circuit is used."]
    _0 = 0,
    #[doc = "1: An SCL synchronous circuit is used."]
    _1 = 1,
}
impl From<SCLE_A> for bool {
    #[inline(always)]
    fn from(variant: SCLE_A) -> Self {
        variant as u8 != 0
    }
}
impl SCLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCLE_A {
        match self.bits {
            false => SCLE_A::_0,
            true => SCLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SCLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SCLE_A::_1
    }
}
#[doc = "Field `SCLE` writer - SCL Synchronous Circuit Enable"]
pub type SCLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ICFER_SPEC, SCLE_A, O>;
impl<'a, const O: u8> SCLE_W<'a, O> {
    #[doc = "No SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SCLE_A::_0)
    }
    #[doc = "An SCL synchronous circuit is used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SCLE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timeout Function Enable"]
    #[inline(always)]
    pub fn tmoe(&self) -> TMOE_R {
        TMOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn male(&self) -> MALE_R {
        MALE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn nale(&self) -> NALE_R {
        NALE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    pub fn sale(&self) -> SALE_R {
        SALE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    pub fn nacke(&self) -> NACKE_R {
        NACKE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    pub fn nfe(&self) -> NFE_R {
        NFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    pub fn scle(&self) -> SCLE_R {
        SCLE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmoe(&mut self) -> TMOE_W<0> {
        TMOE_W::new(self)
    }
    #[doc = "Bit 1 - Master Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn male(&mut self) -> MALE_W<1> {
        MALE_W::new(self)
    }
    #[doc = "Bit 2 - NACK Transmission Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nale(&mut self) -> NALE_W<2> {
        NALE_W::new(self)
    }
    #[doc = "Bit 3 - Slave Arbitration-Lost Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sale(&mut self) -> SALE_W<3> {
        SALE_W::new(self)
    }
    #[doc = "Bit 4 - NACK Reception Transfer Suspension Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nacke(&mut self) -> NACKE_W<4> {
        NACKE_W::new(self)
    }
    #[doc = "Bit 5 - Digital Noise Filter Circuit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfe(&mut self) -> NFE_W<5> {
        NFE_W::new(self)
    }
    #[doc = "Bit 6 - SCL Synchronous Circuit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scle(&mut self) -> SCLE_W<6> {
        SCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Bus Function Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icfer](index.html) module"]
pub struct ICFER_SPEC;
impl crate::RegisterSpec for ICFER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [icfer::R](R) reader structure"]
impl crate::Readable for ICFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icfer::W](W) writer structure"]
impl crate::Writable for ICFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICFER to value 0x72"]
impl crate::Resettable for ICFER_SPEC {
    const RESET_VALUE: Self::Ux = 0x72;
}
