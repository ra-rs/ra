#[doc = "Register `ICUSARE` reader"]
pub struct R(crate::R<ICUSARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICUSARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICUSARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICUSARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICUSARE` writer"]
pub struct W(crate::W<ICUSARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICUSARE_SPEC>;
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
impl From<crate::W<ICUSARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICUSARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAIWDTWUP` reader - Security attributes of registers for WUPEN0.b16"]
pub type SAIWDTWUP_R = crate::BitReader<SAIWDTWUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b16\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIWDTWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIWDTWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAIWDTWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIWDTWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIWDTWUP_A {
        match self.bits {
            false => SAIWDTWUP_A::_0,
            true => SAIWDTWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIWDTWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIWDTWUP_A::_1
    }
}
#[doc = "Field `SAIWDTWUP` writer - Security attributes of registers for WUPEN0.b16"]
pub type SAIWDTWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SAIWDTWUP_A, O>;
impl<'a, const O: u8> SAIWDTWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIWDTWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIWDTWUP_A::_1)
    }
}
#[doc = "Field `SAKEYWUP` reader - Security attributes of registers for WUPEN0.b17"]
pub type SAKEYWUP_R = crate::BitReader<SAKEYWUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b17\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAKEYWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAKEYWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAKEYWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAKEYWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAKEYWUP_A {
        match self.bits {
            false => SAKEYWUP_A::_0,
            true => SAKEYWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAKEYWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAKEYWUP_A::_1
    }
}
#[doc = "Field `SAKEYWUP` writer - Security attributes of registers for WUPEN0.b17"]
pub type SAKEYWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SAKEYWUP_A, O>;
impl<'a, const O: u8> SAKEYWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAKEYWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAKEYWUP_A::_1)
    }
}
#[doc = "Field `SALVD1WUP` reader - Security attributes of registers for WUPEN0.b18"]
pub type SALVD1WUP_R = crate::BitReader<SALVD1WUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b18\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SALVD1WUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SALVD1WUP_A> for bool {
    #[inline(always)]
    fn from(variant: SALVD1WUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SALVD1WUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SALVD1WUP_A {
        match self.bits {
            false => SALVD1WUP_A::_0,
            true => SALVD1WUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SALVD1WUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SALVD1WUP_A::_1
    }
}
#[doc = "Field `SALVD1WUP` writer - Security attributes of registers for WUPEN0.b18"]
pub type SALVD1WUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SALVD1WUP_A, O>;
impl<'a, const O: u8> SALVD1WUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SALVD1WUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SALVD1WUP_A::_1)
    }
}
#[doc = "Field `SALVD2WUP` reader - Security attributes of registers for WUPEN0.b19"]
pub type SALVD2WUP_R = crate::BitReader<SALVD2WUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b19\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SALVD2WUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SALVD2WUP_A> for bool {
    #[inline(always)]
    fn from(variant: SALVD2WUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SALVD2WUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SALVD2WUP_A {
        match self.bits {
            false => SALVD2WUP_A::_0,
            true => SALVD2WUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SALVD2WUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SALVD2WUP_A::_1
    }
}
#[doc = "Field `SALVD2WUP` writer - Security attributes of registers for WUPEN0.b19"]
pub type SALVD2WUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SALVD2WUP_A, O>;
impl<'a, const O: u8> SALVD2WUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SALVD2WUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SALVD2WUP_A::_1)
    }
}
#[doc = "Field `SAAGT1UDWUP` reader - Security attributes of registers for WUPEN0.b28"]
pub type SAAGT1UDWUP_R = crate::BitReader<SAAGT1UDWUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b28\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAAGT1UDWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAAGT1UDWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAAGT1UDWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAAGT1UDWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAAGT1UDWUP_A {
        match self.bits {
            false => SAAGT1UDWUP_A::_0,
            true => SAAGT1UDWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAAGT1UDWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAAGT1UDWUP_A::_1
    }
}
#[doc = "Field `SAAGT1UDWUP` writer - Security attributes of registers for WUPEN0.b28"]
pub type SAAGT1UDWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SAAGT1UDWUP_A, O>;
impl<'a, const O: u8> SAAGT1UDWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAAGT1UDWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAAGT1UDWUP_A::_1)
    }
}
#[doc = "Field `SAAGT1CAWUP` reader - Security attributes of registers for WUPEN0.b29"]
pub type SAAGT1CAWUP_R = crate::BitReader<SAAGT1CAWUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b29\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAAGT1CAWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAAGT1CAWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAAGT1CAWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAAGT1CAWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAAGT1CAWUP_A {
        match self.bits {
            false => SAAGT1CAWUP_A::_0,
            true => SAAGT1CAWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAAGT1CAWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAAGT1CAWUP_A::_1
    }
}
#[doc = "Field `SAAGT1CAWUP` writer - Security attributes of registers for WUPEN0.b29"]
pub type SAAGT1CAWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SAAGT1CAWUP_A, O>;
impl<'a, const O: u8> SAAGT1CAWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAAGT1CAWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAAGT1CAWUP_A::_1)
    }
}
#[doc = "Field `SAAGT1CBWUP` reader - Security attributes of registers for WUPEN0.b30"]
pub type SAAGT1CBWUP_R = crate::BitReader<SAAGT1CBWUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b30\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAAGT1CBWUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAAGT1CBWUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAAGT1CBWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAAGT1CBWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAAGT1CBWUP_A {
        match self.bits {
            false => SAAGT1CBWUP_A::_0,
            true => SAAGT1CBWUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAAGT1CBWUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAAGT1CBWUP_A::_1
    }
}
#[doc = "Field `SAAGT1CBWUP` writer - Security attributes of registers for WUPEN0.b30"]
pub type SAAGT1CBWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SAAGT1CBWUP_A, O>;
impl<'a, const O: u8> SAAGT1CBWUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAAGT1CBWUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAAGT1CBWUP_A::_1)
    }
}
#[doc = "Field `SAIIC0WUP` reader - Security attributes of registers for WUPEN0.b31"]
pub type SAIIC0WUP_R = crate::BitReader<SAIIC0WUP_A>;
#[doc = "Security attributes of registers for WUPEN0.b31\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIIC0WUP_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<SAIIC0WUP_A> for bool {
    #[inline(always)]
    fn from(variant: SAIIC0WUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIIC0WUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAIIC0WUP_A {
        match self.bits {
            false => SAIIC0WUP_A::_0,
            true => SAIIC0WUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAIIC0WUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAIIC0WUP_A::_1
    }
}
#[doc = "Field `SAIIC0WUP` writer - Security attributes of registers for WUPEN0.b31"]
pub type SAIIC0WUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICUSARE_SPEC, SAIIC0WUP_A, O>;
impl<'a, const O: u8> SAIIC0WUP_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAIIC0WUP_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAIIC0WUP_A::_1)
    }
}
impl R {
    #[doc = "Bit 16 - Security attributes of registers for WUPEN0.b16"]
    #[inline(always)]
    pub fn saiwdtwup(&self) -> SAIWDTWUP_R {
        SAIWDTWUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Security attributes of registers for WUPEN0.b17"]
    #[inline(always)]
    pub fn sakeywup(&self) -> SAKEYWUP_R {
        SAKEYWUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Security attributes of registers for WUPEN0.b18"]
    #[inline(always)]
    pub fn salvd1wup(&self) -> SALVD1WUP_R {
        SALVD1WUP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Security attributes of registers for WUPEN0.b19"]
    #[inline(always)]
    pub fn salvd2wup(&self) -> SALVD2WUP_R {
        SALVD2WUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 28 - Security attributes of registers for WUPEN0.b28"]
    #[inline(always)]
    pub fn saagt1udwup(&self) -> SAAGT1UDWUP_R {
        SAAGT1UDWUP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Security attributes of registers for WUPEN0.b29"]
    #[inline(always)]
    pub fn saagt1cawup(&self) -> SAAGT1CAWUP_R {
        SAAGT1CAWUP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Security attributes of registers for WUPEN0.b30"]
    #[inline(always)]
    pub fn saagt1cbwup(&self) -> SAAGT1CBWUP_R {
        SAAGT1CBWUP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Security attributes of registers for WUPEN0.b31"]
    #[inline(always)]
    pub fn saiic0wup(&self) -> SAIIC0WUP_R {
        SAIIC0WUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Security attributes of registers for WUPEN0.b16"]
    #[inline(always)]
    #[must_use]
    pub fn saiwdtwup(&mut self) -> SAIWDTWUP_W<16> {
        SAIWDTWUP_W::new(self)
    }
    #[doc = "Bit 17 - Security attributes of registers for WUPEN0.b17"]
    #[inline(always)]
    #[must_use]
    pub fn sakeywup(&mut self) -> SAKEYWUP_W<17> {
        SAKEYWUP_W::new(self)
    }
    #[doc = "Bit 18 - Security attributes of registers for WUPEN0.b18"]
    #[inline(always)]
    #[must_use]
    pub fn salvd1wup(&mut self) -> SALVD1WUP_W<18> {
        SALVD1WUP_W::new(self)
    }
    #[doc = "Bit 19 - Security attributes of registers for WUPEN0.b19"]
    #[inline(always)]
    #[must_use]
    pub fn salvd2wup(&mut self) -> SALVD2WUP_W<19> {
        SALVD2WUP_W::new(self)
    }
    #[doc = "Bit 28 - Security attributes of registers for WUPEN0.b28"]
    #[inline(always)]
    #[must_use]
    pub fn saagt1udwup(&mut self) -> SAAGT1UDWUP_W<28> {
        SAAGT1UDWUP_W::new(self)
    }
    #[doc = "Bit 29 - Security attributes of registers for WUPEN0.b29"]
    #[inline(always)]
    #[must_use]
    pub fn saagt1cawup(&mut self) -> SAAGT1CAWUP_W<29> {
        SAAGT1CAWUP_W::new(self)
    }
    #[doc = "Bit 30 - Security attributes of registers for WUPEN0.b30"]
    #[inline(always)]
    #[must_use]
    pub fn saagt1cbwup(&mut self) -> SAAGT1CBWUP_W<30> {
        SAAGT1CBWUP_W::new(self)
    }
    #[doc = "Bit 31 - Security attributes of registers for WUPEN0.b31"]
    #[inline(always)]
    #[must_use]
    pub fn saiic0wup(&mut self) -> SAIIC0WUP_W<31> {
        SAIIC0WUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Controller Unit Security Attribution Register E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icusare](index.html) module"]
pub struct ICUSARE_SPEC;
impl crate::RegisterSpec for ICUSARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icusare::R](R) reader structure"]
impl crate::Readable for ICUSARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icusare::W](W) writer structure"]
impl crate::Writable for ICUSARE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICUSARE to value 0xffff_ffff"]
impl crate::Resettable for ICUSARE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
