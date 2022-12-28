#[doc = "Register `BST` reader"]
pub struct R(crate::R<BST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BST` writer"]
pub struct W(crate::W<BST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BST_SPEC>;
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
impl From<crate::W<BST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCNDDF` reader - START Condition Detection Flag"]
pub type STCNDDF_R = crate::BitReader<STCNDDF_A>;
#[doc = "START Condition Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STCNDDF_A {
    #[doc = "0: START condition is not detected."]
    _0 = 0,
    #[doc = "1: START condition is detected."]
    _1 = 1,
}
impl From<STCNDDF_A> for bool {
    #[inline(always)]
    fn from(variant: STCNDDF_A) -> Self {
        variant as u8 != 0
    }
}
impl STCNDDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STCNDDF_A {
        match self.bits {
            false => STCNDDF_A::_0,
            true => STCNDDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STCNDDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STCNDDF_A::_1
    }
}
#[doc = "Field `STCNDDF` writer - START Condition Detection Flag"]
pub type STCNDDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BST_SPEC, STCNDDF_A, O>;
impl<'a, const O: u8> STCNDDF_W<'a, O> {
    #[doc = "START condition is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STCNDDF_A::_0)
    }
    #[doc = "START condition is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STCNDDF_A::_1)
    }
}
#[doc = "Field `SPCNDDF` reader - STOP Condition Detection Flag"]
pub type SPCNDDF_R = crate::BitReader<SPCNDDF_A>;
#[doc = "STOP Condition Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPCNDDF_A {
    #[doc = "0: STOP condition is not detected."]
    _0 = 0,
    #[doc = "1: STOP condition is detected."]
    _1 = 1,
}
impl From<SPCNDDF_A> for bool {
    #[inline(always)]
    fn from(variant: SPCNDDF_A) -> Self {
        variant as u8 != 0
    }
}
impl SPCNDDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPCNDDF_A {
        match self.bits {
            false => SPCNDDF_A::_0,
            true => SPCNDDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPCNDDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPCNDDF_A::_1
    }
}
#[doc = "Field `SPCNDDF` writer - STOP Condition Detection Flag"]
pub type SPCNDDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BST_SPEC, SPCNDDF_A, O>;
impl<'a, const O: u8> SPCNDDF_W<'a, O> {
    #[doc = "STOP condition is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPCNDDF_A::_0)
    }
    #[doc = "STOP condition is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPCNDDF_A::_1)
    }
}
#[doc = "Field `NACKDF` reader - NACK Detection Flag"]
pub type NACKDF_R = crate::BitReader<NACKDF_A>;
#[doc = "NACK Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKDF_A {
    #[doc = "0: NACK is not detected."]
    _0 = 0,
    #[doc = "1: NACK is detected."]
    _1 = 1,
}
impl From<NACKDF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKDF_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKDF_A {
        match self.bits {
            false => NACKDF_A::_0,
            true => NACKDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKDF_A::_1
    }
}
#[doc = "Field `NACKDF` writer - NACK Detection Flag"]
pub type NACKDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BST_SPEC, NACKDF_A, O>;
impl<'a, const O: u8> NACKDF_W<'a, O> {
    #[doc = "NACK is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NACKDF_A::_0)
    }
    #[doc = "NACK is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NACKDF_A::_1)
    }
}
#[doc = "Field `TENDF` reader - Transmit End Flag"]
pub type TENDF_R = crate::BitReader<TENDF_A>;
#[doc = "Transmit End Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TENDF_A {
    #[doc = "0: Data is being transmitted."]
    _0 = 0,
    #[doc = "1: Data has been transmitted."]
    _1 = 1,
}
impl From<TENDF_A> for bool {
    #[inline(always)]
    fn from(variant: TENDF_A) -> Self {
        variant as u8 != 0
    }
}
impl TENDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TENDF_A {
        match self.bits {
            false => TENDF_A::_0,
            true => TENDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TENDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TENDF_A::_1
    }
}
#[doc = "Field `TENDF` writer - Transmit End Flag"]
pub type TENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BST_SPEC, TENDF_A, O>;
impl<'a, const O: u8> TENDF_W<'a, O> {
    #[doc = "Data is being transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TENDF_A::_0)
    }
    #[doc = "Data has been transmitted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TENDF_A::_1)
    }
}
#[doc = "Field `ALF` reader - Arbitration Lost Flag"]
pub type ALF_R = crate::BitReader<ALF_A>;
#[doc = "Arbitration Lost Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALF_A {
    #[doc = "0: Arbitration is not lost"]
    _0 = 0,
    #[doc = "1: Arbitration is lost."]
    _1 = 1,
}
impl From<ALF_A> for bool {
    #[inline(always)]
    fn from(variant: ALF_A) -> Self {
        variant as u8 != 0
    }
}
impl ALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALF_A {
        match self.bits {
            false => ALF_A::_0,
            true => ALF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALF_A::_1
    }
}
#[doc = "Field `ALF` writer - Arbitration Lost Flag"]
pub type ALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BST_SPEC, ALF_A, O>;
impl<'a, const O: u8> ALF_W<'a, O> {
    #[doc = "Arbitration is not lost"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALF_A::_0)
    }
    #[doc = "Arbitration is lost."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALF_A::_1)
    }
}
#[doc = "Field `TODF` reader - Timeout Detection Flag"]
pub type TODF_R = crate::BitReader<TODF_A>;
#[doc = "Timeout Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TODF_A {
    #[doc = "0: Timeout is not detected."]
    _0 = 0,
    #[doc = "1: Timeout is detected."]
    _1 = 1,
}
impl From<TODF_A> for bool {
    #[inline(always)]
    fn from(variant: TODF_A) -> Self {
        variant as u8 != 0
    }
}
impl TODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TODF_A {
        match self.bits {
            false => TODF_A::_0,
            true => TODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TODF_A::_1
    }
}
#[doc = "Field `TODF` writer - Timeout Detection Flag"]
pub type TODF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BST_SPEC, TODF_A, O>;
impl<'a, const O: u8> TODF_W<'a, O> {
    #[doc = "Timeout is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TODF_A::_0)
    }
    #[doc = "Timeout is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TODF_A::_1)
    }
}
#[doc = "Field `WUCNDDF` reader - Wake-Up Condition Detection Flag"]
pub type WUCNDDF_R = crate::BitReader<WUCNDDF_A>;
#[doc = "Wake-Up Condition Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUCNDDF_A {
    #[doc = "0: Wake-Up is not detected."]
    _0 = 0,
    #[doc = "1: Wake-Up is detected."]
    _1 = 1,
}
impl From<WUCNDDF_A> for bool {
    #[inline(always)]
    fn from(variant: WUCNDDF_A) -> Self {
        variant as u8 != 0
    }
}
impl WUCNDDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUCNDDF_A {
        match self.bits {
            false => WUCNDDF_A::_0,
            true => WUCNDDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUCNDDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUCNDDF_A::_1
    }
}
#[doc = "Field `WUCNDDF` writer - Wake-Up Condition Detection Flag"]
pub type WUCNDDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, BST_SPEC, WUCNDDF_A, O>;
impl<'a, const O: u8> WUCNDDF_W<'a, O> {
    #[doc = "Wake-Up is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUCNDDF_A::_0)
    }
    #[doc = "Wake-Up is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUCNDDF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - START Condition Detection Flag"]
    #[inline(always)]
    pub fn stcnddf(&self) -> STCNDDF_R {
        STCNDDF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP Condition Detection Flag"]
    #[inline(always)]
    pub fn spcnddf(&self) -> SPCNDDF_R {
        SPCNDDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - NACK Detection Flag"]
    #[inline(always)]
    pub fn nackdf(&self) -> NACKDF_R {
        NACKDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit End Flag"]
    #[inline(always)]
    pub fn tendf(&self) -> TENDF_R {
        TENDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Arbitration Lost Flag"]
    #[inline(always)]
    pub fn alf(&self) -> ALF_R {
        ALF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Timeout Detection Flag"]
    #[inline(always)]
    pub fn todf(&self) -> TODF_R {
        TODF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake-Up Condition Detection Flag"]
    #[inline(always)]
    pub fn wucnddf(&self) -> WUCNDDF_R {
        WUCNDDF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - START Condition Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stcnddf(&mut self) -> STCNDDF_W<0> {
        STCNDDF_W::new(self)
    }
    #[doc = "Bit 1 - STOP Condition Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn spcnddf(&mut self) -> SPCNDDF_W<1> {
        SPCNDDF_W::new(self)
    }
    #[doc = "Bit 4 - NACK Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nackdf(&mut self) -> NACKDF_W<4> {
        NACKDF_W::new(self)
    }
    #[doc = "Bit 8 - Transmit End Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tendf(&mut self) -> TENDF_W<8> {
        TENDF_W::new(self)
    }
    #[doc = "Bit 16 - Arbitration Lost Flag"]
    #[inline(always)]
    #[must_use]
    pub fn alf(&mut self) -> ALF_W<16> {
        ALF_W::new(self)
    }
    #[doc = "Bit 20 - Timeout Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn todf(&mut self) -> TODF_W<20> {
        TODF_W::new(self)
    }
    #[doc = "Bit 24 - Wake-Up Condition Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wucnddf(&mut self) -> WUCNDDF_W<24> {
        WUCNDDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bst](index.html) module"]
pub struct BST_SPEC;
impl crate::RegisterSpec for BST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bst::R](R) reader structure"]
impl crate::Readable for BST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bst::W](W) writer structure"]
impl crate::Writable for BST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BST to value 0"]
impl crate::Resettable for BST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
