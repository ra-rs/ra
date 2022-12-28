#[doc = "Register `ECSIPR` reader"]
pub struct R(crate::R<ECSIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECSIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECSIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECSIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECSIPR` writer"]
pub struct W(crate::W<ECSIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECSIPR_SPEC>;
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
impl From<crate::W<ECSIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECSIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICDIP` reader - False Carrier Detect Interrupt Enable"]
pub type ICDIP_R = crate::BitReader<ICDIP_A>;
#[doc = "False Carrier Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICDIP_A {
    #[doc = "0: Notification of the false carrier detect interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Notification of the false carrier detect interrupt is enabled."]
    _1 = 1,
}
impl From<ICDIP_A> for bool {
    #[inline(always)]
    fn from(variant: ICDIP_A) -> Self {
        variant as u8 != 0
    }
}
impl ICDIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICDIP_A {
        match self.bits {
            false => ICDIP_A::_0,
            true => ICDIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICDIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICDIP_A::_1
    }
}
#[doc = "Field `ICDIP` writer - False Carrier Detect Interrupt Enable"]
pub type ICDIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSIPR_SPEC, ICDIP_A, O>;
impl<'a, const O: u8> ICDIP_W<'a, O> {
    #[doc = "Notification of the false carrier detect interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICDIP_A::_0)
    }
    #[doc = "Notification of the false carrier detect interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICDIP_A::_1)
    }
}
#[doc = "Field `MPDIP` reader - Magic Packet Detect Interrupt Enable"]
pub type MPDIP_R = crate::BitReader<MPDIP_A>;
#[doc = "Magic Packet Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPDIP_A {
    #[doc = "0: Notification of the Magic Packet detect interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Notification of the Magic Packet detect interrupt is enabled."]
    _1 = 1,
}
impl From<MPDIP_A> for bool {
    #[inline(always)]
    fn from(variant: MPDIP_A) -> Self {
        variant as u8 != 0
    }
}
impl MPDIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPDIP_A {
        match self.bits {
            false => MPDIP_A::_0,
            true => MPDIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPDIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPDIP_A::_1
    }
}
#[doc = "Field `MPDIP` writer - Magic Packet Detect Interrupt Enable"]
pub type MPDIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSIPR_SPEC, MPDIP_A, O>;
impl<'a, const O: u8> MPDIP_W<'a, O> {
    #[doc = "Notification of the Magic Packet detect interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MPDIP_A::_0)
    }
    #[doc = "Notification of the Magic Packet detect interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MPDIP_A::_1)
    }
}
#[doc = "Field `LCHNGIP` reader - LINK Signal Change Interrupt Enable"]
pub type LCHNGIP_R = crate::BitReader<LCHNGIP_A>;
#[doc = "LINK Signal Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCHNGIP_A {
    #[doc = "0: Notification of ETn_LINKSTA signal change interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Notification of ETn_LINKSTA signal change interrupt is enabled."]
    _1 = 1,
}
impl From<LCHNGIP_A> for bool {
    #[inline(always)]
    fn from(variant: LCHNGIP_A) -> Self {
        variant as u8 != 0
    }
}
impl LCHNGIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCHNGIP_A {
        match self.bits {
            false => LCHNGIP_A::_0,
            true => LCHNGIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LCHNGIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LCHNGIP_A::_1
    }
}
#[doc = "Field `LCHNGIP` writer - LINK Signal Change Interrupt Enable"]
pub type LCHNGIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSIPR_SPEC, LCHNGIP_A, O>;
impl<'a, const O: u8> LCHNGIP_W<'a, O> {
    #[doc = "Notification of ETn_LINKSTA signal change interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LCHNGIP_A::_0)
    }
    #[doc = "Notification of ETn_LINKSTA signal change interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LCHNGIP_A::_1)
    }
}
#[doc = "Field `PSRTOIP` reader - PAUSE Frame Retransmit Over Interrupt Enable"]
pub type PSRTOIP_R = crate::BitReader<PSRTOIP_A>;
#[doc = "PAUSE Frame Retransmit Over Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSRTOIP_A {
    #[doc = "0: Notification of PAUSE frame retransmit over interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Notification of PAUSE frame retransmit over interrupt is enabled."]
    _1 = 1,
}
impl From<PSRTOIP_A> for bool {
    #[inline(always)]
    fn from(variant: PSRTOIP_A) -> Self {
        variant as u8 != 0
    }
}
impl PSRTOIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSRTOIP_A {
        match self.bits {
            false => PSRTOIP_A::_0,
            true => PSRTOIP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSRTOIP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSRTOIP_A::_1
    }
}
#[doc = "Field `PSRTOIP` writer - PAUSE Frame Retransmit Over Interrupt Enable"]
pub type PSRTOIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSIPR_SPEC, PSRTOIP_A, O>;
impl<'a, const O: u8> PSRTOIP_W<'a, O> {
    #[doc = "Notification of PAUSE frame retransmit over interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSRTOIP_A::_0)
    }
    #[doc = "Notification of PAUSE frame retransmit over interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSRTOIP_A::_1)
    }
}
#[doc = "Field `BFSIPR` reader - Continuous Broadcast Frame Reception Interrupt Enable"]
pub type BFSIPR_R = crate::BitReader<BFSIPR_A>;
#[doc = "Continuous Broadcast Frame Reception Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFSIPR_A {
    #[doc = "0: Notification of continuous broadcast frame reception interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Notification of continuous broadcast frame reception interrupt is enabled."]
    _1 = 1,
}
impl From<BFSIPR_A> for bool {
    #[inline(always)]
    fn from(variant: BFSIPR_A) -> Self {
        variant as u8 != 0
    }
}
impl BFSIPR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFSIPR_A {
        match self.bits {
            false => BFSIPR_A::_0,
            true => BFSIPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BFSIPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BFSIPR_A::_1
    }
}
#[doc = "Field `BFSIPR` writer - Continuous Broadcast Frame Reception Interrupt Enable"]
pub type BFSIPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECSIPR_SPEC, BFSIPR_A, O>;
impl<'a, const O: u8> BFSIPR_W<'a, O> {
    #[doc = "Notification of continuous broadcast frame reception interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BFSIPR_A::_0)
    }
    #[doc = "Notification of continuous broadcast frame reception interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BFSIPR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - False Carrier Detect Interrupt Enable"]
    #[inline(always)]
    pub fn icdip(&self) -> ICDIP_R {
        ICDIP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Detect Interrupt Enable"]
    #[inline(always)]
    pub fn mpdip(&self) -> MPDIP_R {
        MPDIP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LINK Signal Change Interrupt Enable"]
    #[inline(always)]
    pub fn lchngip(&self) -> LCHNGIP_R {
        LCHNGIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PAUSE Frame Retransmit Over Interrupt Enable"]
    #[inline(always)]
    pub fn psrtoip(&self) -> PSRTOIP_R {
        PSRTOIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Continuous Broadcast Frame Reception Interrupt Enable"]
    #[inline(always)]
    pub fn bfsipr(&self) -> BFSIPR_R {
        BFSIPR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - False Carrier Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icdip(&mut self) -> ICDIP_W<0> {
        ICDIP_W::new(self)
    }
    #[doc = "Bit 1 - Magic Packet Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpdip(&mut self) -> MPDIP_W<1> {
        MPDIP_W::new(self)
    }
    #[doc = "Bit 2 - LINK Signal Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lchngip(&mut self) -> LCHNGIP_W<2> {
        LCHNGIP_W::new(self)
    }
    #[doc = "Bit 4 - PAUSE Frame Retransmit Over Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psrtoip(&mut self) -> PSRTOIP_W<4> {
        PSRTOIP_W::new(self)
    }
    #[doc = "Bit 5 - Continuous Broadcast Frame Reception Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfsipr(&mut self) -> BFSIPR_W<5> {
        BFSIPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETHERC Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecsipr](index.html) module"]
pub struct ECSIPR_SPEC;
impl crate::RegisterSpec for ECSIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecsipr::R](R) reader structure"]
impl crate::Readable for ECSIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecsipr::W](W) writer structure"]
impl crate::Writable for ECSIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECSIPR to value 0"]
impl crate::Resettable for ECSIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
