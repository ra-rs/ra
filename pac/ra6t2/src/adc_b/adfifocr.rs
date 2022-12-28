#[doc = "Register `ADFIFOCR` reader"]
pub struct R(crate::R<ADFIFOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFIFOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFIFOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFIFOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFIFOCR` writer"]
pub struct W(crate::W<ADFIFOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFIFOCR_SPEC>;
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
impl From<crate::W<ADFIFOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFIFOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOEN0` reader - Scan Group n FIFO Enable"]
pub type FIFOEN0_R = crate::BitReader<FIFOEN0_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN0_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN0_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN0_A {
        match self.bits {
            false => FIFOEN0_A::_0,
            true => FIFOEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN0_A::_1
    }
}
#[doc = "Field `FIFOEN0` writer - Scan Group n FIFO Enable"]
pub type FIFOEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN0_A, O>;
impl<'a, const O: u8> FIFOEN0_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN0_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN0_A::_1)
    }
}
#[doc = "Field `FIFOEN1` reader - Scan Group n FIFO Enable"]
pub type FIFOEN1_R = crate::BitReader<FIFOEN1_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN1_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN1_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN1_A {
        match self.bits {
            false => FIFOEN1_A::_0,
            true => FIFOEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN1_A::_1
    }
}
#[doc = "Field `FIFOEN1` writer - Scan Group n FIFO Enable"]
pub type FIFOEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN1_A, O>;
impl<'a, const O: u8> FIFOEN1_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN1_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN1_A::_1)
    }
}
#[doc = "Field `FIFOEN2` reader - Scan Group n FIFO Enable"]
pub type FIFOEN2_R = crate::BitReader<FIFOEN2_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN2_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN2_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN2_A {
        match self.bits {
            false => FIFOEN2_A::_0,
            true => FIFOEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN2_A::_1
    }
}
#[doc = "Field `FIFOEN2` writer - Scan Group n FIFO Enable"]
pub type FIFOEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN2_A, O>;
impl<'a, const O: u8> FIFOEN2_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN2_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN2_A::_1)
    }
}
#[doc = "Field `FIFOEN3` reader - Scan Group n FIFO Enable"]
pub type FIFOEN3_R = crate::BitReader<FIFOEN3_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN3_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN3_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN3_A {
        match self.bits {
            false => FIFOEN3_A::_0,
            true => FIFOEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN3_A::_1
    }
}
#[doc = "Field `FIFOEN3` writer - Scan Group n FIFO Enable"]
pub type FIFOEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN3_A, O>;
impl<'a, const O: u8> FIFOEN3_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN3_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN3_A::_1)
    }
}
#[doc = "Field `FIFOEN4` reader - Scan Group n FIFO Enable"]
pub type FIFOEN4_R = crate::BitReader<FIFOEN4_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN4_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN4_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN4_A {
        match self.bits {
            false => FIFOEN4_A::_0,
            true => FIFOEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN4_A::_1
    }
}
#[doc = "Field `FIFOEN4` writer - Scan Group n FIFO Enable"]
pub type FIFOEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN4_A, O>;
impl<'a, const O: u8> FIFOEN4_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN4_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN4_A::_1)
    }
}
#[doc = "Field `FIFOEN5` reader - Scan Group n FIFO Enable"]
pub type FIFOEN5_R = crate::BitReader<FIFOEN5_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN5_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN5_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN5_A {
        match self.bits {
            false => FIFOEN5_A::_0,
            true => FIFOEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN5_A::_1
    }
}
#[doc = "Field `FIFOEN5` writer - Scan Group n FIFO Enable"]
pub type FIFOEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN5_A, O>;
impl<'a, const O: u8> FIFOEN5_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN5_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN5_A::_1)
    }
}
#[doc = "Field `FIFOEN6` reader - Scan Group n FIFO Enable"]
pub type FIFOEN6_R = crate::BitReader<FIFOEN6_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN6_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN6_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN6_A {
        match self.bits {
            false => FIFOEN6_A::_0,
            true => FIFOEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN6_A::_1
    }
}
#[doc = "Field `FIFOEN6` writer - Scan Group n FIFO Enable"]
pub type FIFOEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN6_A, O>;
impl<'a, const O: u8> FIFOEN6_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN6_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN6_A::_1)
    }
}
#[doc = "Field `FIFOEN7` reader - Scan Group n FIFO Enable"]
pub type FIFOEN7_R = crate::BitReader<FIFOEN7_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN7_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN7_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN7_A {
        match self.bits {
            false => FIFOEN7_A::_0,
            true => FIFOEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN7_A::_1
    }
}
#[doc = "Field `FIFOEN7` writer - Scan Group n FIFO Enable"]
pub type FIFOEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN7_A, O>;
impl<'a, const O: u8> FIFOEN7_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN7_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN7_A::_1)
    }
}
#[doc = "Field `FIFOEN8` reader - Scan Group n FIFO Enable"]
pub type FIFOEN8_R = crate::BitReader<FIFOEN8_A>;
#[doc = "Scan Group n FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN8_A {
    #[doc = "0: Disable scan group n FIFO function"]
    _0 = 0,
    #[doc = "1: Enable scan group n FIFO function"]
    _1 = 1,
}
impl From<FIFOEN8_A> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN8_A) -> Self {
        variant as u8 != 0
    }
}
impl FIFOEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFOEN8_A {
        match self.bits {
            false => FIFOEN8_A::_0,
            true => FIFOEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIFOEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIFOEN8_A::_1
    }
}
#[doc = "Field `FIFOEN8` writer - Scan Group n FIFO Enable"]
pub type FIFOEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADFIFOCR_SPEC, FIFOEN8_A, O>;
impl<'a, const O: u8> FIFOEN8_W<'a, O> {
    #[doc = "Disable scan group n FIFO function"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIFOEN8_A::_0)
    }
    #[doc = "Enable scan group n FIFO function"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIFOEN8_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen0(&self) -> FIFOEN0_R {
        FIFOEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen1(&self) -> FIFOEN1_R {
        FIFOEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen2(&self) -> FIFOEN2_R {
        FIFOEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen3(&self) -> FIFOEN3_R {
        FIFOEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen4(&self) -> FIFOEN4_R {
        FIFOEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen5(&self) -> FIFOEN5_R {
        FIFOEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen6(&self) -> FIFOEN6_R {
        FIFOEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen7(&self) -> FIFOEN7_R {
        FIFOEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Group n FIFO Enable"]
    #[inline(always)]
    pub fn fifoen8(&self) -> FIFOEN8_R {
        FIFOEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen0(&mut self) -> FIFOEN0_W<0> {
        FIFOEN0_W::new(self)
    }
    #[doc = "Bit 1 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen1(&mut self) -> FIFOEN1_W<1> {
        FIFOEN1_W::new(self)
    }
    #[doc = "Bit 2 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen2(&mut self) -> FIFOEN2_W<2> {
        FIFOEN2_W::new(self)
    }
    #[doc = "Bit 3 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen3(&mut self) -> FIFOEN3_W<3> {
        FIFOEN3_W::new(self)
    }
    #[doc = "Bit 4 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen4(&mut self) -> FIFOEN4_W<4> {
        FIFOEN4_W::new(self)
    }
    #[doc = "Bit 5 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen5(&mut self) -> FIFOEN5_W<5> {
        FIFOEN5_W::new(self)
    }
    #[doc = "Bit 6 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen6(&mut self) -> FIFOEN6_W<6> {
        FIFOEN6_W::new(self)
    }
    #[doc = "Bit 7 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen7(&mut self) -> FIFOEN7_W<7> {
        FIFOEN7_W::new(self)
    }
    #[doc = "Bit 8 - Scan Group n FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen8(&mut self) -> FIFOEN8_W<8> {
        FIFOEN8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adfifocr](index.html) module"]
pub struct ADFIFOCR_SPEC;
impl crate::RegisterSpec for ADFIFOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adfifocr::R](R) reader structure"]
impl crate::Readable for ADFIFOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adfifocr::W](W) writer structure"]
impl crate::Writable for ADFIFOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFIFOCR to value 0"]
impl crate::Resettable for ADFIFOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
